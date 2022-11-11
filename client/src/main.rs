use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone, PartialEq, Deserialize)]
struct User {
    name: String,
}

fn create_random_users(n: usize) -> Vec<User> {
    let mut users: Vec<User> = Vec::new();

    for _ in 0..n {
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        users.push(User { name: s.to_owned() });
    }
    users
}

fn main() {
    let cpus = num_cpus::get();
    let users = create_random_users(cpus * 2);


    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(cpus)
        .enable_all()
        .build()
        .unwrap();

    for _ in 0..1000 {
        let handles: Vec<tokio::task::JoinHandle<_>> = users
            .iter()
            .map(|u| rt.spawn(create_unit(u.clone())))
            .collect();

        rt.block_on(async {
            for handle in handles {
                handle.await.unwrap();
            }
        });
    }
}

async fn create_unit(user: User) {
    let client = reqwest::Client::new();
    // println!("Sending: {:?}", user);
    let response = client
        .post("http://localhost:3000/user")
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&user)
        .send()
        .await
        .expect("err1")
        .text()
        .await
        .expect("err2");

    let recv_user: User = serde_json::from_str(&response).unwrap();

    // println!("Receiving: {:?}", recv_user);
}
