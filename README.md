# mysql_layer

This is a project for making crud operations on mysql. The intention was to create a database trait that allow any object to easily implement requests like get/put/post/delete automatically by simply implement a trait on an object.

I intended to use this project in a serverless lambda application for aws. With this foundation done, any lambda function could be implemented automatically when a user requested it. It would run faster and more securely than the usual nodejs implementations. If a lambda function requires 10x less computation to run, that means it will cost 10x less for a company to maintain.

But, I just found out that rust mysql's library doesn't compile into webassembly. My alternative to that was calling the function from nodejs. So currently I'm learning how to configure nodejs, webpack and wasmbindgen properly.

Folder structure is explained below:

 ## bin

Contains the executable binaries for running tests for the request. This was made just to quickly see if functions to mysql worked properly. Unit tests will be implemented later.

 ## cli

Command line interface arguments module. It retrieves a string of arguments acquired from calling the functions. It worked when I built it first, but subsequent changes to the project made me not implement arguments for the binaries. Therefore not being used at the moment.

 ## database

Module that holds everything related to the mysql interface. 

 - "link" file accesses the .env file and holds the information that "db" uses to connect to the database.

 - "db" has all the functions to directly interact with the database. Built this way so that dao interacts with a rust module, not directly to mysql. That way the compiler assures code safety. An extra module layer will be optimized by the compiler, so there will be close to no overhead.

 - "dao" is a trait that uses "db" to bind the data to the database. The user tells the module what are the columns for given object and how to parse them. The module handles the rest.

 - "errors" is a module where I would enumerate all the possible errors that might occur from the database. Each error would have it's own statuscode and message. A lot easier to debug. But mysql doesn't compile to webassembly, so now I'm still figuring out what to do next.

 ## requests

This module is what would interact with the serverless framework. Both by parsing the request and sending the response.  

 - "gateway" is not used here. It's a module to parse aws http events. I was going to use it in the serverless project.

 - "request" has the trait that selects the function based on what request type has been received. That way we could have get/post/put/delete requests all going to the same endpoint.

 - "response" has all the most used status codes and offers a very simple way for a developer to retrieve a response. This entire module will be optimized away by the compiler. So all these 300 lines of code take 0 processing time and occupy 4 bytes of memory.

 ## lambda

This is where the lambda functions will be stored. Each folder represents a type of data.

 - I moved "message" from this folder into the requests folder in my serverless project. Will do it later here too. This type handles json serializing/deserializing. It stores a String json from an object or another String. After stored, we can attempt to deserialize the String into the message we want to retrieve. If it's valid return Ok(value), else Err(err).

 - "funcionario" means "employee". I manually implemented get/put/post/delete functions for this type just to see if they work. Now I noticed that we can use generic types instead. That way we only need to write around 80 lines of code to implement all crud operation in a object, having it work perfectly within an aws lambda framework.

 ## "starting_db.txt" file

This is a sketch file that I was using for figuring out how to implement environment variables and test connection through a mysql docker instance. Kept it there just for shows.

# Environment variables

Environment variables are used for connecting to mysql. I deleted personally identifiable ip from the .env, but examples are still there.

# The Big Problem

Since rust's mysql library doesn't compile into webassembly. The whole project hit a roadblock. I have two options:

 - I can call mysql queries through a function implemented in nodejs using webassembly, that would run a bit less fast, and be more complex. 
 
 - Or implement the entire lambda project in rust. Which would be clunkier because there still isn't proper automated ways of uploading rust into lambda. I would have to manually upload the zip folders. Maybe there could be a way to automate it using jenkins. But I would have to research it.
