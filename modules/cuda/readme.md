# [cuda](https://en.wikipedia.org/wiki/CUDA)

CUDA computing platform for nvidia cards

## Container Toolkit

> https://docs.nvidia.com/dgx/nvidia-container-runtime-upgrade/

To support NVIDIA GPUs used by GPU-accelerated containers

## bitsandbytes

> Assuming you have python installed

If a program requires bitsandbytes it's best to compile it locally with the detected cuda version.

Clone bitsandbytes

```sh
git clone https://github.com/TimDettmers/bitsandbytes
cd bitsandbytes
```

Then make and install using pip **while being in the `venv` of the
tool you're using**.

> You can omit the `CUDA_VERSION` environmental variable when using this
> dotmodule as it already defines it in `17-cuda.conf`

```sh
CUDA_VERSION=121 make cuda11x
pip install .
```

> ! You have to reinstall it everytime the target tool overwrites it with a
> ! non-custom built version of bitsandbytes. It's best to completely take it
> ! out of thier requirement.txt and just do it yourself.
