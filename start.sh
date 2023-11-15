#!/bin/bash

# 下载并执行 foundry.paradigm.xyz 脚本
curl -L https://foundry.paradigm.xyz | bash

# 执行foundryup
/bin/bash -c "/root/.foundry/bin/foundryup"

# 将foundry相关命令移动到/bin目录
mv /root/.foundry/bin/cast /bin
mv /root/.foundry/bin/foundryup /bin
mv /root/.foundry/bin/anvil /bin
mv /root/.foundry/bin/forge /bin

./tg_bot