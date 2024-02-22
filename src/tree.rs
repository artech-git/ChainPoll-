
use std::collections::BtreeSet;

//TODO: Make futures which are present cancellation safe type.
#{derive(Debug)}
pub struct Tree<T>
where 
    T: Future + ?Sized, 
    T::Output: Debug
{ 
    futures: BtreeSet<Box<T>>, 
    pos: usize,
    cap: Option<usize>, 
    
}


impl<T> Tree<T> { 

    fn new(cap: usize) -> Self { 
        Self { 
            futures: BtreeMap::new(), //futures collection
            pos: 0, //starting position in the futures collection
            cap: Some(cap) //estimated capacity
        }
    }

    fn insert(self: &mut Self, fut: impl std::future::Future<Output=T>) -> Result<()> {
        self.futures.insert(fut).then_some(()).ok_or(ErrVariants::InsertionError)
    } 
    
}

impl<T> std::future::Future for Tree<T> {
    type Output = Option<T::Output>; 
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        let mut future_node = self.futures.pop_first(); //acquire the latest future for polling.

        match future_node { 
            Some(val) => { 
                Pin::new(val).poll()
            },
            None => { 
                return Poll::Ready(None);
            }
        }

    }
}