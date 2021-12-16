use std::{thread, time};
use tokio::task::spawn_blocking;

#[tokio::main]
async fn main() {
    let unsorted = [6, 7, 3, 5, 1, 7, 9];

    for i in unsorted {
        spawn_blocking(move || {
            let mils = time::Duration::from_millis(i * 10);
            thread::sleep(mils);
            println!("{}", i);
        });
    }
}
u s e   s t d : : { e n v : : a r g s ,   t h r e a d ,   t i m e } ;  
 u s e   t o k i o : : t a s k : : s p a w n _ b l o c k i n g ;  
  
 # [ t o k i o : : m a i n ]  
 a s y n c   f n   m a i n ( )   {  
         l e t   m u t   a r g v   =   a r g s ( ) . c o l l e c t : : < V e c < S t r i n g > > ( ) ;  
         a r g v . r e m o v e ( 0 ) ;  
          
         l e t   n u m b e r s   =   a r g v . i t e r ( ) . m a p ( | x |   {  
                 x . p a r s e : : < u 6 4 > ( ) . e x p e c t ( " Y o u   p r o v i d e d   a n   i n v a l i d   n u m b e r . " )  
         } ) ;  
  
         f o r   i   i n   n u m b e r s   {  
                 s p a w n _ b l o c k i n g ( m o v e   | |   {  
                         l e t   m i l s   =   t i m e : : D u r a t i o n : : f r o m _ m i l l i s ( i   *   1 0 ) ;  
                         t h r e a d : : s l e e p ( m i l s ) ;  
                         p r i n t l n ! ( " { } " ,   i ) ;  
                 } ) ;  
         }  
 }  
 