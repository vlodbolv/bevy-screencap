distrobox create \
  --name bevy-dev \
  --image registry.fedoraproject.org/fedora-toolbox:40 \
  --volume "$HOME/Code:$HOME/Code" \
  --additional-flags "--device=/dev/dri" \
  --yes
