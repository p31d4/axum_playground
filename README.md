# Repository to play with Rust Axum

You can use the <em>build_and_run.sh</em> script to start an axum web app from this repo.  
For example:
```
bash ./build_and_run.sh hello_axum
```

Once the Docker container is running you can access the application with an HTTP client.  
For example in your browser:
```
127.0.0.1:3000/hello
```

or from the command line:
```
curl http://127.0.0.1:3000/hello
```

To stop the Docker container open a new terminal and type the following commands:
```
docker ps
docker stop <CONTAINER ID>
```
