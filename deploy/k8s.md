# Kubernetes

## Cluster

![cluster](assets/module_01_cluster.svg)

We can use `minikube` to create(mock maybe?) a Cluster, and the cluster will have one node named `minikube`.

## Node

Every Kubernetes Node runs at least:

- **Kubelet**, a process responsible for communication between the Kubernetes control plane and the Node; it manages the Pods and the containers running on a machine.
- **A container runtime** (like Docker) responsible for pulling the container image from a registry, unpacking the container, and running the application.

Node module:

![nodes](assets/module_03_nodes.svg)

## Pod

Pod Lifecycle:

![pod](assets/pod.svg)

Once the scheduler assigns a Pod to a Node, the Node’s `kubelet` ensures the Pod’s containers match the Pod’s config with the container runtime. 

## Deployment

The Deployment `instructs` Kubernetes how to create and update instances of your application.

> An available Pod is an **instance** that is available to the users of the application.

![deployment](assets/module_02_first_app.svg)

```sh
$ kubectl create deployment kubernetes-bootcamp --image=gcr.io/google-samples/kubernetes-bootcamp:v1
```

The deployment did these:

- find and run an instance of the application on a suitable node
- **scheduled** the application to run on that Node
- configured the cluster to reschedule the instance on a new Node **when needed**

So, the deployment is used to run the instances and ensure them working as expected.

## Service

A `Service` abstracts a logical set of Pods and a policy to access the pods by. In other words. `Service` is used to expose the application.

The set of Pods of a Service is usually determined by a `label selector`.

Each of pods has an ip, but they are not exposed without Service. Service allow the pods to recv traffic. There are serveral ways: 

- ClusterIP
- NodePort
- LoadBalancer
- ExternalName

Services match a set of Pods using Label Selectors. 

### New Service

```sh
$ kubectl get pods
NAME                                  READY   STATUS              RESTARTS   AGE
kubernetes-bootcamp-f95c5b745-xksgr   0/1     ContainerCreating   0          4s

$ kubectl get services  
NAME         TYPE        CLUSTER-IP   EXTERNAL-IP   PORT(S)   AGE
kubernetes   ClusterIP   10.96.0.1    <none>        443/TCP   3m46s

$ kubectl expose deployment/kubernetes-bootcamp --type="NodePort" --port 8080       
service/kubernetes-bootcamp exposed

$ kubectl get services
NAME                  TYPE        CLUSTER-IP      EXTERNAL-IP   PORT(S)          AGE
kubernetes            ClusterIP   10.96.0.1       <none>        443/TCP          8m51s
kubernetes-bootcamp   NodePort    10.102.92.185   <none>        8080:31402/TCP   4m53s
```

Test:

```sh
$ curl http://"$(minikube ip):31402"
```

If docker as runtime, open a new terminal and follow this way, a minikube tunnel is needed.

```sh
$ minikube service kubernetes-bootcamp --url
http://127.0.0.1:63733
$ curl http://127.0.0.1:63733
Hello Kubernetes bootcamp! | Running on: kubernetes-bootcamp-f95c5b745-xksgr | v=1
```

### Label

```sh
$ kubectl label pods "$POD_NAME" key=value
$ kubectl get pods -l key=value
```

### Delete Service

```sh
$ kubectl delete service -l app=kubernetes-bootcamp
```

## Scale

Above, the Service just has one Pod(Instance). We can add more in the Service so that a Service could expose more Pods, which is **not** limited on the same Node.

Scaling out a Deployment will ensure new Pods are created and scheduled to Nodes with available resources.

Auto scaling is also supported.

Moreover, running multiple instances of an application will require a way to **distribute the traffic** to all of them. 

```sh
$ kubectl get deployments
NAME                  READY   UP-TO-DATE   AVAILABLE   AGE
kubernetes-bootcamp   1/1     1            1           69m
```

Here’s one Pod. Then: 

```sh
$ kubectl get rs
NAME                            DESIRED   CURRENT   READY   AGE
kubernetes-bootcamp-f95c5b745   1         1         1       70m
```

`rs` is short for `ReplicaSet`.

### Scale up

Then scale the deployments:

```sh
$ kubectl scale deployments/kubernetes-bootcamp --replicas=4
deployment.apps/kubernetes-bootcamp scaled
$ kubectl get deployments
NAME                  READY   UP-TO-DATE   AVAILABLE   AGE
kubernetes-bootcamp   4/4     4            4           73m
```

Ready: 4/4

Now we can view the deployment’s describe:

```sh
$ kubectl describe deployments/kubernetes-bootcamp
...
Replicas: 4 desired | 4 updated | 4 total | 4 available | 0 unavailable
```

Test:

```sh
$ curl http://127.0.0.1:63733
Hello Kubernetes bootcamp! | Running on: kubernetes-bootcamp-f95c5b745-xksgr | v=1
$ curl http://127.0.0.1:63733
Hello Kubernetes bootcamp! | Running on: kubernetes-bootcamp-f95c5b745-l4z25 | v=1
$ curl http://127.0.0.1:63733
Hello Kubernetes bootcamp! | Running on: kubernetes-bootcamp-f95c5b745-xksgr | v=1
$ curl http://127.0.0.1:63733
Hello Kubernetes bootcamp! | Running on: kubernetes-bootcamp-f95c5b745-l75qj | v=1
```

The resps are different, so that here’s a load balance.

### Scale down

```sh
$ kubectl scale deployments/kubernetes-bootcamp --replicas=2
deployment.apps/kubernetes-bootcamp scaled
$ kubectl get deployments
NAME                  READY   UP-TO-DATE   AVAILABLE   AGE
kubernetes-bootcamp   2/2     2            2           94m
```

## Roll Update

**Rolling updates** allow Deployments' update to perform without downtime by incrementally updating Pods instances with new ones. 

By default, only one old instance will be replaced by a new one at each moment, which means only one Pod unavailable during the update. And this is configurable. 

Last , updates are versioned and can be reverted.

Rolling updates allow the following actions:

- Promote an application from one environment to another (via container image updates)
- Rollback to previous versions
- Continuous Integration and Continuous Delivery (CI/CD) of applications with zero downtime



Use the `set image` subcommand, followed by the deployment name and the new image version:

```sh
$ kubectl get deployments
NAME                  READY   UP-TO-DATE   AVAILABLE   AGE
kubernetes-bootcamp   2/2     2            2           113m
$ kubectc get pods
NAME                                  READY   STATUS    RESTARTS   AGE
kubernetes-bootcamp-f95c5b745-l75qj   1/1     Running   0          38m
kubernetes-bootcamp-f95c5b745-xksgr   1/1     Running   0          110m
$ kubectl describe pods
...
Image: gcr.io/google-samples/kubernetes-bootcamp:v1
```

### Update version


```sh
$ kubectl set image deployments/kubernetes-bootcamp kubernetes-bootcamp=jocatalin/kubernetes-bootcamp:v2
deployment.apps/kubernetes-bootcamp image updated
```

### Verify

```sh
$ minikube service kubernetes-bootcamp --url
http://127.0.0.1:64456
❗  Because you are using a Docker driver on darwin, the terminal needs to be open to run it.
$ curl http://127.0.0.1:64456
Hello Kubernetes bootcamp! | Running on: kubernetes-bootcamp-65df967b7f-5vjz5 | v=2
```

Also:

```sh
$ kubectl rollout status deployments/kubernetes-bootcamp
deployment "kubernetes-bootcamp" successfully rolled out
```

Or: 

```sh
$ kubectl describe pods
...
Image: jocatalin/kubernetes-bootcamp:v2
```

### Roll back

Perform a wrong update:

```sh
$ kubectl set image deployments/kubernetes-bootcamp kubernetes-bootcamp=gcr.io/google-samples/kubernetes-bootcamp:v10
$ kubectl get pods
NAME                                   READY   STATUS         RESTARTS   AGE
...
kubernetes-bootcamp-7497bc6797-bdgbx   0/1     ErrImagePull   0          38s
```

Check reason:

```sh
$ kubectl describe pods/kubernetes-bootcamp-7497bc6797-bdgbx
...
Events:
...
Failed to pull image "gcr.io/google-samples/kubernetes-bootcamp:v10": Error response from daemon: manifest for gcr.io/google-samples/kubernetes-bootcamp:v10 not found: manifest unknown: Failed to fetch "v10" from request "/v2/google-samples/kubernetes-bootcamp/manifests/v10".
```

To undo the last operation:

```sh
$ kubectl rollout undo deployments/kubernetes-bootcamp
deployment.apps/kubernetes-bootcamp rolled back
```

## Clean Cluster

```sh
$ kubectl delete deployments/kubernetes-bootcamp services/kubernetes-bootcamp
$ kubectl get svc
NAME         TYPE        CLUSTER-IP   EXTERNAL-IP   PORT(S)   AGE
kubernetes   ClusterIP   10.96.0.1    <none>        443/TCP   133m
$ kubectl get deploy
No resources found in default namespace.
$ kubectl get pods
No resources found in default namespace.
```











Some other subcommands:

- `kubectl get` - list resources
- `kubectl describe` - show detailed information about a resource
- `kubectl logs` - print the logs from a container in a pod
- `kubectl exec` - execute a command on a container in a pod



