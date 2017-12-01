# -*- mode: ruby -*-
# vi: set ft=ruby :

# Vagrantfile API/syntax version. Don't touch unless you know what you're doing!
VAGRANTFILE_API_VERSION = "2"

Vagrant.configure(VAGRANTFILE_API_VERSION) do |config|
    # Set box for all VMs to Ubuntu cloud image
    config.vm.box = "ubuntu-xenial"
    config.vm.box_url = "https://cloud-images.ubuntu.com/xenial/current/xenial-server-cloudimg-amd64-vagrant.box"
    
    config.vm.provision "shell", inline: <<-SHELL
      apt-get update
      apt-get install -y build-essential curl
    SHELL

    config.vm.provision "shell", privileged: false, inline: <<-RUST
        # install rust compiler
        curl https://sh.rustup.rs -sSf | sh -s -- -y
    RUST
end
