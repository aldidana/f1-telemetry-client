# f1-telemetry-client

> 🏎️ Telemetry client for F1 game by Codemasters 🏎️

## Example
```rust
use f1_telemetry_client::{Telemetry, packet::Packet};
use async_std::task;

fn main() {
    task::block_on(async {
        let telemetry = Telemetry::new("192.168.1.11", 20777).await.unwrap();
        
        loop {
            match telemetry.next().await {
                Ok(packet) => {
                    match packet {
                        Packet::F12020(result) => {
                            println!("Result {:?}", result);
                        }
                        _ => unimplemented!(),
                    }
                },
                Err(e) => {
                    eprintln!("Error {}", e)
                }
            }
        }
    })
}
```

### Enable Telemetry Setting
<img width="712" alt="web-checkssl" src="https://user-images.githubusercontent.com/6572635/97430345-5a1ca380-194b-11eb-929f-99012adb699e.png">

### UDP Specifications
- 2020 - https://forums.codemasters.com/topic/50942-f1-2020-udp-specification/

### Credits for struct
- https://github.com/mathieu-lemay/f1-telemetry-rs/
- https://github.com/hellobits/f1-api/

### License
MIT @Aldi Priya Perdana