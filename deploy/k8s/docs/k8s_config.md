# k8s configuration

## Config

4 ways to config environment variables:

- Dockerfile
- kubernetes.yml
- Kubernetes ConfigMaps 
- Kubernetes Secrets

### Kubernetes ConfigMaps

key value

```yaml
# example-redis-config.yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: example-redis-config
data:
  redis-config: ""
EOF
```

Apply it:

```sh
$ kubectl apply -f example-redis-config.yaml
$ kubectl apply -f https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/pods/config/redis-pod.yaml
```

Check the redis-pod.yaml

```yaml
# redis-pod.yaml
apiVersion: v1
kind: Pod
metadata:
  name: redis
spec:
  containers:
  - name: redis
    image: redis:5.0.4
    command:
      - redis-server
      - "/redis-master/redis.conf"
    env:
    - name: MASTER
      value: "true"
    ports:
    - containerPort: 6379
    resources:
      limits:
        cpu: "0.1"
    volumeMounts:
    - mountPath: /redis-master-data
      name: data
    - mountPath: /redis-master
      name: config
  volumes:
    - name: data
      emptyDir: {}
    - name: config
      configMap:
        name: example-redis-config # exposes the example-redis-config ConfigMap's
        items:
        - key: redis-config # redis-config key
          path: redis.conf # as a file named redis.conf on the config volume
```

Check the ConfigMap in Kubernetes:

```sh
$ kubectl describe configmap/example-redis-config
Name:         example-redis-config
...
Data
====
redis-config:
```

The redis-config is empty.

Check the Redis:

```sh
$ kubectl exec -it redis -- redis-cli
127.0.0.1:6379> CONFIG GET maxmemory

1) "maxmemory"
2) "0"
```

So let’s update the config to:

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: example-redis-config
data:
  redis-config: |
    maxmemory 2mb
    maxmemory-policy allkeys-lru    
```

ps: yaml’s `|` means keep the format of the string, the same as `r#""#` in rust.

Then apply the new configmap:

```sh
$ kubectl apply -f example-redis-config.yaml
$ kubectl describe configmap/example-redis-config
Data
====
redis-config:
----
maxmemory 2mb
maxmemory-policy allkeys-lru

# should restart the pod to apply the new config
$ kubectl delete pods/redis
$ kubectl apply -f https://raw.githubusercontent.com/kubernetes/website/main/content/en/examples/pods/config/redis-pod.yaml
```

### Secrets

base64 encoding

