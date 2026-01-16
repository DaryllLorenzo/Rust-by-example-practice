use std::process::Output;
use tokio::time::sleep;
use std::time::Duration;

pub struct tk {}


impl tk {
    pub async fn run(){
        println!("------ EX9 async/await -------");

        let mut handles = vec![];

        for i in 0..2 {
            let h = tokio::spawn(async move {
                tk::read_from_database(i).await;
            });

            handles.push(h);
        }

        for h in handles {
            h.await.unwrap();
        }
    }

    /* Its the same 
    fn run () -> impl Future<Output = ()>{
        async {
            
        }
    }
    */

    async fn read_from_database(i: i32){
        sleep(Duration::from_secs(1)).await;
        println!("Reading data {}", i);
        sleep(Duration::from_secs(1)).await;
        println!("Returning data {}", i);
    }
}
