use binary_heap_plus::BinaryHeap;

fn compare_tags<A>((x,_): &(u32,A), (y,_): &(u32,A)) -> std::cmp::Ordering {
    x.cmp(y)
}

pub fn find_smallest<A>(
    items: &[A],
    get_priority: impl Fn(&A) -> u32,
    num_items: usize,
    always_keep_below: u32)
    -> Vec<(u32, &A)>
{
    let mut heap = BinaryHeap::with_capacity_by(num_items, compare_tags);
    for item in items {
        let sz: u32 = get_priority(item);
        if heap.len() < num_items {
            heap.push((sz, item));
        } else {
            let worst = heap.peek().unwrap().0;
            if sz < worst {
                if worst >= always_keep_below {
                    heap.pop();
                }
                heap.push((sz,item));
            }
        }
    }

    heap.into_sorted_vec()
}