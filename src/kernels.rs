use super::{Node, Solved, Movement};
use std::thread;
use std::sync::mpsc;

pub struct MsgToSlave {
    node: Option<Box<Node>>,
}

pub struct MsgToMaster {
    node: Box<Node>,
    childs: Vec<Box<Node>>,
    thread_id: usize,
}

pub fn slave(tx: mpsc::Sender<MsgToMaster>, rx: mpsc::Receiver<MsgToSlave>, thread_id: usize) {
    loop {
        let job = rx.recv().unwrap();
        let mut node = match job.node {
            Some(value) => value,
            None => return (),
        };
        let childs = (*node).get_childs();
        tx.send(MsgToMaster {
            node: node,
            thread_id: thread_id,
            childs: childs,
        });
    }
}

fn push_sorted(openset: &mut Vec<Box<Node>>, node: Box<Node>) {
    let index = openset.binary_search(&node).unwrap_or_else(|e| e);
    openset.insert(index, node);
}

pub fn master(mut start: Node, rx: mpsc::Receiver<MsgToMaster>, txs: Vec<mpsc::Sender<MsgToSlave>>) -> Result<Solved, &'static str> {
    let mut openset = Vec::<Box<Node>>::new();
    let mut closeset = Vec::<Box<Node>>::new();
    let mut status: bool = true;
    let h: u16;

    let mut memory: usize = 0;
    let mut complexity: usize = 0;

    if let Some(ref mut map) = start.map {
        map.display();
        map.translate_in();
        map.check_validity()?;
        map.set_first_costs();
        h = map.get_cost();
    } else {
        return Err("Got a weird thing happening...");
    }

    start.h = h;
    start.f = h;
    openset.push(Box::new(start));

    for _ in 0..(txs.len() * 2) { // Doing some calculation on only one thread to feed the openset
        let mut node = openset.pop().unwrap();
        txs[0].send(MsgToSlave {
            node: Some(node)
        });
        let response = rx.recv().unwrap();
        let parent = closeset.len();
        let mut childs = response.childs;

        closeset.push(response.node);
        while childs.len() > 0 {
            let mut child = childs.pop().unwrap();
            (*child).parent = parent;
            if (*child).h == 0 {
                closeset.push(child);
                status = false;
                break;
            }
            push_sorted(&mut openset, child);
            complexity += 1;
        }
        if closeset.len() + openset.len() > memory {
            memory = closeset.len() + openset.len();
        }
        if status == false {
            break;
        }
    }

    if status {
        for i in 0..txs.len() {
            let mut node = openset.pop().unwrap();
            txs[i].send(MsgToSlave {
                node: Some(node)
            });
        }
        while status {
            let response = rx.recv().unwrap();
            let parent = closeset.len();
            let mut childs = response.childs;

            closeset.push(response.node);
            while childs.len() > 0 {
                let mut child = childs.pop().unwrap();
                (*child).parent = parent;
                if (*child).h == 0 {
                    closeset.push(child);
                    status = false;
                    break;
                }
                push_sorted(&mut openset, child);
                complexity += 1;
            }

            if closeset.len() + openset.len() > memory {
                memory = closeset.len() + openset.len();
            }

            let node = openset.pop().unwrap();
            txs[response.thread_id].send(MsgToSlave {
                node: Some(node),
            });
        }
    }

    for tx in txs.iter() {
        tx.send(MsgToSlave { node: None });
    }

    let mut sequence: Vec<Movement> = Vec::<Movement>::new();
    let mut id = closeset.len();
    let mut node = closeset.pop().unwrap();
    sequence.push((*node).movement);

    while id > 0 {
        id = (*node).parent;
        node = closeset.remove(id);
        sequence.push((*node).movement);
    }

    //println!("{:#?}", openset); // Dumping openset for analysis

    Ok(Solved {
        memory: memory,
        complexity: complexity,
        sequence: sequence
    })
}

pub fn process(start: Node, nthreads: usize) -> Result<Solved, &'static str> {
    let mut slaves = vec![];
    let mut trx                   : Vec<(mpsc::Sender<MsgToSlave>, mpsc::Receiver<MsgToSlave>)> = (0..nthreads).map(|_| mpsc::channel()).collect();
    let (tx_report, rx_report): (mpsc::Sender<MsgToMaster>, mpsc::Receiver<MsgToMaster>)    = mpsc::channel();
    let mut tx_whips          : Vec<mpsc::Sender<MsgToSlave>>                                = vec![];


    for id in 0..nthreads {
        let (tx_whip, rx_whip) = trx.pop().unwrap();
        let tx_report_copy = tx_report.clone();

        slaves.push(thread::spawn(move || {
            slave(tx_report_copy, rx_whip, id);
        }));
        tx_whips.push(tx_whip);
    }
    let res = master(start, rx_report, tx_whips);
    while slaves.len() > 0 {
        let slave = slaves.pop().unwrap();
        slave.join();
    }
    res
}
