
## Create it from Dockerfile
```
docker build -t topological-sort .
```


#### Create and start your container Linux
```
docker run -ti --name  topological-sort  -v $(pwd)/topological-sort:/topological-sort  --init topological-sort bash
```
docker run -ti --name  topological-sort  -v $(pwd)/topological-sort:/topological-sort bash
### 7. ###
To stop your session: press CTRL+C twice and ENTER in your terminal window. Then type `exit`. 
Once out of the container, type
```
docker stop topological-sort
```
### 8. ### 
To resume your session, just type:
```
docker start  topological-sort
docker exec -ti  topological-sort sh
 