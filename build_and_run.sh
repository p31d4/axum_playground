#!/bin/bash

PROJECT_DIR="hello_axum"

if [ "$#" -gt 0  ]
then
    PROJECT_DIR=${1}
fi

docker build -t ${PROJECT_DIR} --no-cache --build-arg PROJECT=${PROJECT_DIR} . 
docker run --rm -p 3000:3000 ${PROJECT_DIR} -c "/${PROJECT_DIR}"
