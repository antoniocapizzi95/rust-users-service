# rust-user-service

This is a small personal project developed in Rust with Actix and MongoDB to expose CRUD operations for a User entity. It has the following endpoints:

    GET /users/{id}
    GET /users
    POST /users
    PUT /users/{id}
    DELETE /users/{id}

## Installation

To run this project, you need to have Docker and Docker Compose installed on your machine.

1. Clone this repository
2. Navigate to the project directory
3. Run ```docker-compose up```

This will start two containers: one for the Rust application and one for MongoDB.

## API Endpoints

### Get User

This endpoint returns a user by ID.

Request

    GET /users/{id}

Parameters

    id (string) - The ID of the user.

Response

    200 OK - If the user was found.
    404 Not Found - If the user was not found.

    Body example:
    {
        "id": "123",
        "name": "John",
        "surname": "Doe",
        "city": "New York"
    }

### Get Users

This endpoint returns a list of all users.

Request

    GET /users

Response

    200 OK - If the users were found.

    Body example:
    [
        {
            "id": "123",
            "name": "John",
            "surname": "Doe",
            "city": "New York"
        },
        {
            "id": "456",
            "name": "Jane",
            "surname": "Doe",
            "city": "Los Angeles"
        }
    ]

### Create User

This endpoint creates a new user.

Request

    POST /users

    Body

    {
        "id": "123",
        "name": "John",
        "surname": "Doe",
        "city": "New York"
    }

Response

    201 Created - If the user was created.

    Body:
    {
        "id": "123",
        "name": "John",
        "surname": "Doe",
        "city": "New York"
    }

### Update User
This endpoint updates an existing user.

Request

    PUT /users/{id}

Parameters

    id (string) - The ID of the user.
    Body
    
    {
    "name": "John",
    "surname": "Doe",
    "city": "Los Angeles"
    }

Response

    200 OK - If the user was updated.
    404 Not Found - If the user was not found.

    Body example:
    {
    "id": "123",
    "name": "John",
    "surname": "Doe",
    "city": "Los Angeles"
    }

### Delete User
This endpoint deletes an existing user.
Request

    DELETE /users/{id}
Parameters

    id (string) - The ID of the user.
Response

    204 No Content - If the user was deleted.
    404 Not Found - If the user was not found.