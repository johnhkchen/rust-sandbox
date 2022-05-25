# rust-sandbox

Just a simple repo demonstrating running a rust package through Docker.

cd hello_world
docker build -t my-rust-app .
docker run -it --rm --name my-running-app my-rust-app