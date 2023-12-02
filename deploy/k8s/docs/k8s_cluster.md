This is about how to establish a k8s cluster.

## Network

Ensure these ports being open

https://kubernetes.io/docs/reference/networking/ports-and-protocols/

## Container Runtime

A container runtime like docker.

## Installing kubeadm, kubelet and kubectl

- `kubeadm`: the command to bootstrap the cluster.
- `kubelet`: the component that runs on all of the machines in your cluster and does things like starting pods and containers.
- `kubectl`: the command line util to talk to your cluster.

[how]( https://kubernetes.io/docs/setup/production-environment/tools/kubeadm/install-kubeadm/#installing-kubeadm-kubelet-and-kubectl)

## Configuring a cgroup driver

Control Group (cgroup) is a feature in the Linux kernel that allows you to limit, account for, and isolate resource usage of process groups. Cgroups can be used to restrict CPU usage, memory usage, network bandwidth, and more, making it valuable in containerization and virtualization scenarios.

Includ systemd:

```yaml
# kubeadm-config.yaml
kind: ClusterConfiguration
apiVersion: kubeadm.k8s.io/v1beta3
kubernetesVersion: v1.21.0
---
kind: KubeletConfiguration
apiVersion: kubelet.config.k8s.io/v1beta1
cgroupDriver: systemd
```

## Create a Cluster with kubeadm

[more](https://kubernetes.io/docs/setup/production-environment/tools/kubeadm/create-cluster-kubeadm/)

### Prepare the Host

- components installation (finished in the front)
- network setup
- required images (default pull from registry.k8s.io)
- initialize control-plane node

#### network

kubeadm similarly to other Kubernetes components tries to find a usable IP on the network interfaces associated with a default gateway on a host.

> "Gateway" typically refers to the default gateway in the network, which is a network device or router used to route packets from one subnet (or local network) to another subnet or the internet.
>
> ```sh
> ip route show # Look for a line starting with "default via"
> ```

#### initialize

The control-plane node is the machine where the control plane components run, including [etcd](https://kubernetes.io/docs/tasks/administer-cluster/configure-upgrade-etcd/) (the cluster database) and the [API Server](https://kubernetes.io/docs/concepts/overview/components/#kube-apiserver) (which the [kubectl](https://kubernetes.io/docs/reference/kubectl/) command line tool communicates with).

- --control-plane-endpoint (if need upgrading this single control-plane `kubeadm`cluster to high availability)
- --pod-network-cidr ([Installing a Pod network add-on](https://kubernetes.io/docs/setup/production-environment/tools/kubeadm/create-cluster-kubeadm/#pod-network))
- --cri-socket (choose different container runtime)

```sh
kubeadm init <args>
```

- --apiserver-advertise-address (being used to set the advertise address for this particular control-plane node's API server)
- --control-plane-endpoint (being used to set the shared endpoint for all control-plane nodes)

Turning a single control plane cluster created without `--control-plane-endpoint` into a highly available cluster is not supported by kubeadm.

## Cluster Configuration

- `apiServer`
- `controllerManager`
- `scheduler`
- `etcd`

1. Add the appropriate `extraArgs` to your configuration.
2. Add flags to the `extraArgs` field.
3. Run `kubeadm init` with `--config <YOUR CONFIG YAML>`.

Show default configuration:

```sh
kubeadm config print init-defaults
```

flags:

https://kubernetes.io/docs/reference/command-line-tools-reference/kube-apiserver/

https://kubernetes.io/docs/reference/command-line-tools-reference/kube-controller-manager/

https://kubernetes.io/docs/reference/command-line-tools-reference/kube-scheduler/

https://etcd.io/docs/

patches

## Options for Highly Available Topology

You can set up an HA cluster:

- With stacked control plane nodes, where etcd nodes are colocated with control plane nodes
- With external etcd nodes, where etcd runs on separate nodes from the control plane

![stacked control plane nodes](assets/kubeadm-ha-topology-stacked-etcd.svg)

![External etcd topology](assets/kubeadm-ha-topology-external-etcd.svg)

## create

## set up
