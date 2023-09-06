# Hyprland Workspaces across multiple monitors

I Don't like the independant monitors so i made a smol program that didn't need it
## How to use
Install (i use nixos btw) so recommended is currently to use nix


```conf
# Instead of
bind =  $mainMod, 1, workspace 1
# you use
bind = $mainMod, 1, exec, wam workspace 1
```
so the thing that changes is the exec and the wam
