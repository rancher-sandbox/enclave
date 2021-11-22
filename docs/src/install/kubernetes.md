# On Kubernetes

This section explains how to install lockc on a Kubernetes cluster with
[helm](https://helm.sh/).

The helm chart is available in lockc git repository / source code.
Installation with default values can be done with:

```bash
helm install lockc contrib/helm/lockc/ --namespace kube-system
```

To use your own container image, you can override values. Please refer to the
[Container image](../build/container-image.md) section for instructions about
building container images with lockc. Let's assume that you pushed an image
with lockc to `ttl.sh/caa530ed-1371-43f7-a9ad-293a4f930f83:30m`. In that case,
installation with that image can be done with the following command:

```bash
helm install lockc contrib/helm/lockc/ --namespace kube-system \
    --set lockcd.image.repository=ttl.sh/caa530ed-1371-43f7-a9ad-293a4f930f83 \
    --set lockcd.image.tag=30m
```

Enabling debug logs can be helpful for troubleshooting or development. That can
be done with the following command:

```bash
helm install lockc contrib/helm/lockc/ --namespace kube-system \
    --set lockcd.image.repository=ttl.sh/caa530ed-1371-43f7-a9ad-293a4f930f83 \
    --set lockcd.image.tag=30m \
    --set lockcd.debug.enabled=true
```