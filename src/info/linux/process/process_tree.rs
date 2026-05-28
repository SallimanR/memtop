use std::{collections::HashMap, ops::Deref};

use crate::info::linux::process::{self, ProcessType};

#[derive(Debug, Default)]
pub struct ProcessData {
    pub pid: u32,
    pub ppid: u32,
    pub tgid: u32,

    pub name: String,

    pub process_type: ProcessType,
}

#[derive(Debug, Default)]
pub struct TreeNode {
    pub data: ProcessData,
    pub children: Vec<usize>,
}

#[derive(Debug, Default)]
pub struct ProcessTree(pub Vec<TreeNode>);

impl Deref for ProcessTree {
    type Target = Vec<TreeNode>;
    fn deref(&self) -> &Vec<TreeNode> {
        &self.0
    }
}

impl ProcessTree {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn update(&mut self, toggle_threads: bool) -> Option<()> {
        self.build_process_tree(toggle_threads)
    }

    fn build_process_tree(&mut self, toggle_threads: bool) -> Option<()> {
        #[cfg(feature = "profile-with-tracy")]
        let _span = tracy_client::span!("ProcessList::build_process_tree");

        self.0.clear();

        let mut procs: Vec<ProcessData> = Vec::new();
        let mut buf = [0u8; 512];

        let pids = process::get_pids_of_all_processes()?;

        for pid in pids {
            let proc_stat = process::proc_get_stat(pid, &mut buf)?;
            let tgid = process::proc_get_tgid(pid, &mut buf)?;

            let process_type = match proc_stat.is_kernel_thread {
                true => ProcessType::Kernel,
                false => ProcessType::Regular,
            };
            let process = ProcessData {
                pid,
                ppid: proc_stat.ppid,
                tgid,
                name: proc_stat.comm,
                process_type,
            };
            procs.push(process);

            if !toggle_threads {
                continue;
            }

            let threads_ids = process::proc_get_threads_ids(pid)?.skip(1);
            for thread_id in threads_ids {
                let name = process::thread_get_name(pid, thread_id);

                let process = ProcessData {
                    pid: thread_id,
                    ppid: proc_stat.ppid,
                    tgid,
                    process_type: ProcessType::Thread,
                    name,
                };
                procs.push(process);
            }
        }
        debug_assert!(!procs.is_empty());

        let mut pid_to_idx: HashMap<u32, usize> = HashMap::new();
        for (i, p) in procs.iter().enumerate() {
            pid_to_idx.insert(p.pid, i);
        }
        debug_assert!(!pid_to_idx.is_empty());

        let mut nodes: Vec<TreeNode> = procs
            .into_iter()
            .map(|data| TreeNode {
                data,
                children: Vec::new(),
            })
            .collect();
        debug_assert!(!nodes.is_empty());

        for i in 0..nodes.len() {
            let pid = nodes[i].data.pid;
            let ppid = nodes[i].data.ppid;
            let tgid = nodes[i].data.tgid;

            let parent_idx = if pid != tgid {
                // This is a thread: attach to its main process
                pid_to_idx.get(&tgid).copied()
            } else {
                // Regular process: attach to its parent
                pid_to_idx.get(&ppid).copied()
            };

            if let Some(p_idx) = parent_idx {
                nodes[p_idx].children.push(i);
            }
        }

        self.0 = nodes;

        None
    }

    pub fn print_tree(&self, root_indices: &[usize]) {
        fn recurse(nodes: &[TreeNode], idx: usize, depth: usize) {
            let node = &nodes[idx];
            let indent = "  ".repeat(depth);
            println!("{}├─ {} (PID {})", indent, node.data.name, node.data.pid);
            for &child in &node.children {
                recurse(nodes, child, depth + 1);
            }
        }
        for &root_idx in root_indices {
            recurse(&self.0, root_idx, 0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_process_tree() {
        let mut process_tree = ProcessTree::new();
        assert_eq!(process_tree.update(true), None);
        assert!(!process_tree.0.is_empty());
        let root_indices: Vec<usize> = process_tree
            .iter()
            .enumerate()
            .filter(|(_, node)| {
                let ppid = node.data.ppid;
                // if no node has pid == ppid, it's a root
                !process_tree.iter().any(|n| n.data.pid == ppid)
            })
            .map(|(i, _)| i)
            .collect();
        assert!(!root_indices.is_empty());

        process_tree.print_tree(&root_indices);
    }
}
