//Importing Library
use  async_std::task;
use futures;
use surf;
use futures::try_join;

fn main()-> Result<(),Box<dyn std::error::Error + Send + Sync>>{
    task::block_on(
        async{
            let request_1 = surf::get("https://www.facebook.com/").recv_string();
            let request_2 = surf::get("https://www.instagram.com/").recv_string();
            let (response_1,response_2)=futures::future::try_join(request_1,request_2).await? ;
            dbg!("{:?}",response_1);
            dbg!("{:?}",response_2);
            Ok(())       
        } )
}
