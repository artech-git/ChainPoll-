
use std::collections::BTreeSet;
use std::task::Poll;
use std::future::Future; 
use std::pin::Pin; 
use std::fmt::Debug; 

use crate::error::{ChainPollResult, ErrVariants}; 


//TODO: Make futures which are present cancellation safe type.
#[derive(Debug)]
pub struct Tree<T>
where 
    T: Future + ?Sized, 
    T::Output: Debug // modify the trait bounds for supporting meaningful behaviour
{ 
    futures: BTreeSet<Box<T>>, 
    pos: usize,
    cap: Option<usize>
}

impl<T> Tree<T> { 
    fn new(cap: usize) -> Self { 
        Self { 
            futures: BTreeSet::new(), //futures collection
            pos: 0, //starting position in the futures collection
            cap: Some(cap) //estimated capacity
        }
    }

    fn insert(self: &mut Self, fut: impl std::future::Future<Output=T>) -> ChainPollResult<()> {
        self.futures.insert(fut).then_some(()).ok_or(ErrVariants::InsertionError)
    } 
}

impl<T> std::future::Future for Tree<T> {
    type Output = Option<T::Output>; 

    //TODO: ensure that call to the future is Cancel/Drop safe !, if possible ? 
    fn poll(
        self: std::pin::Pin<&mut Self>, 
        cx: &mut std::task::Context<'_>
    ) -> std::task::Poll<Self::Output> {
    
        //acquire the latest future for polling.
        let mut future_node = self.futures.pop_first(); 

        match future_node { 
            Some(val) => { 

                let mut ping_res = Pin::new(& **val).as_mut().poll(cx); 
                
                match fut_res { 
                    Poll::Ready(val) => {
                        self.pos += 1; 
                        return Poll::Ready(Some(val));
                    }, 
                    Poll::Pending => { 
                        return Poll::Pending;
                    }
                }
            },
            None => { 
                return Poll::Ready(None);
            }
        }

    }
}