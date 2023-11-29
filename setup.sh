# !/bin/bash

# reference: https://techblog.ap-com.co.jp/entry/2019/06/28/100439

set -eux

# sudo ip netns add [name] コマンドは, 新しいネットワーク名前空間を作成する
sudo ip netns add host1
sudo ip netns add router
sudo ip netns add host2

# sudo ip link add name [device-name] type veth peer name [peer-device-name] は, ペアの仮想イーサネットデバイスを作成する
# これにより、異なる名前空間間でのネットワーク通信が可能になる
sudo ip link add name host1-veth1 type veth peer name router-veth1
sudo ip link add name router-veth2 type veth peer name host2-veth1

# sudo ip link set [device-name] netns [namespace] は, 特定のデバイスを特定のネットワーク名前空間に割り当てる
sudo ip link set host1-veth1 netns host1
sudo ip link set router-veth1 netns router
sudo ip link set router-veth2 netns router
sudo ip link set host2-veth1 netns host2

# sudo ip netns exec [namespace] ip addr add [ip-address] dev [device-name] は, 特定の名前空間内のデバイスにIPアドレスを割り当てる
sudo ip netns exec host1 ip addr add 10.0.0.1/24 dev host1-veth1
sudo ip netns exec router ip addr add 10.0.0.254/24 dev router-veth1
sudo ip netns exec router ip addr add 10.0.1.254/24 dev router-veth2
sudo ip netns exec host2 ip addr add 10.0.1.1/24 dev host2-veth1

# sudo ip netns exec [namespace] ip link set [device-name] up は, デバイスを有効化する
sudo ip netns exec host1 ip link set host1-veth1 up
sudo ip netns exec router ip link set router-veth1 up
sudo ip netns exec router ip link set router-veth2 up
sudo ip netns exec host2 ip link set host2-veth1 up

# sudo ip netns exec [namespace] ip link set lo up は, 各名前空間のローカルループバックインターフェイスを有効化する
sudo ip netns exec host1 ip link set lo up
sudo ip netns exec router ip link set lo up
sudo ip netns exec host2 ip link set lo up

# sudo ip netns exec [namespace] ip route add 0.0.0.0/0 via [gateway-ip] は, デフォルトゲートウェイを設定する
sudo ip netns exec host1 ip route add 0.0.0.0/0 via 10.0.0.254
sudo ip netns exec host2 ip route add 0.0.0.0/0 via 10.0.1.254

# sudo ip netns exec router sysctl -w net.ipv4.ip_forward=1 は, ルーターとして機能する名前空間でIPフォワーディングを有効化する
sudo ip netns exec router sysctl -w net.ipv4.ip_forward=1

# sudo ip netns exec [namespace] sudo iptables -A OUTPUT -p tcp --tcp-flags RST RST -j DROP は,
# TCP接続のリセットを示すRSTフラグを持つパケットをドロップするルールを追加する
sudo ip netns exec host1 sudo iptables -A OUTPUT -p tcp --tcp-flags RST RST -j DROP
sudo ip netns exec host2 sudo iptables -A OUTPUT -p tcp --tcp-flags RST RST -j DROP

# sudo ip netns exec [namespace] sudo ethtool -K [device-name] tx off は, 特定のデバイスで送信チェックサムオフローディングを無効化する
sudo ip netns exec host2 sudo ethtool -K host2-veth1 tx off
sudo ip netns exec host1 sudo ethtool -K host1-veth1 tx off
