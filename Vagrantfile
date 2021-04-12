# -*- mode: ruby -*-
# vi: set ft=ruby :

Vagrant.configure(2) do |config|
  config.vm.box = "ubuntu/bionic64"

  config.vm.provider "virtualbox" do |v|
    v.memory = 3072
  end

  # install rust and native dependencies
  config.vm.provision "shell", inline: <<-SHELL
    apt-get update
    apt-get install -y build-essential libgtk-3-dev libappindicator3-dev \
                       libssl-dev clang-3.8

    wget --no-verbose -O rustup-init \
      https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init
    chmod a+x rustup-init
    su -c './rustup-init --profile minimal -vy' vagrant
    rm rustup-init
  SHELL
end
