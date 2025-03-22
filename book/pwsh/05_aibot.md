# aibot

`aibot`の構成です。

- https://git.syui.ai/ai/bot

|title|body|
|---|---|
|aibot|ai/bot|
|server|archlinux|
|os|ai/os|
|ai|ai/ai|

```sh
$ winget install ollama.ollama
$ ollama server
$ ollama run llama3.2

$ winget install --id Python.Python.3.11 -e
$ python --version
$ python -m venv webui
$ cd webui
$ .\Scripts\activate
$ pip install open-webui
$ open-webui serve

http://localhost:8080
```

## LoRA

finetuning

```sh
$ conda create -n finetuning python=3.11
$ conda activate finetuning
$ pip install mlx-lm #apple silicon
$ ollama run llama3.2
$ echo "{ \"model\": \"https://huggingface.co/meta-llama/Llama-3.2-3B-Instruct\", \"data\": \"https://github.com/ml-explore/mlx-examples/tree/main/lora/data\" }"|jq .
$ model=meta-llama/Llama-3.2-3B-Instruct
$ data=ml-explore/mlx-examples/lora/data
$ mlx_lm.lora --train --model $model --data $data --batch-size 3

$ ls adapters
$ vim Modelfile
FROM llama3.2:3b
ADAPTER ./adapters

$ ollama create ai -f ./Modelfile
```

## unsloth

```sh
$ pip install unsloth
```

```py
from unsloth import FastLanguageModel

model, tokenizer = FastLanguageModel.from_pretrained(
    model_name="Qwen2.5-1.5B",
    grpo=True
)
```
