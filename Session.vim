let SessionLoad = 1
let s:so_save = &g:so | let s:siso_save = &g:siso | setg so=0 siso=0 | setl so=-1 siso=-1
let v:this_session=expand("<sfile>:p")
silent only
silent tabonly
cd ~/Dock/repos/rust/bevy/jump-game
if expand('%') == '' && !&modified && line('$') <= 1 && getline(1) == ''
  let s:wipebuf = bufnr('%')
endif
let s:shortmess_save = &shortmess
if &shortmess =~ 'A'
  set shortmess=aoOA
else
  set shortmess=aoO
endif
badd +86 ~/Dock/repos/rust/bevy/jump-game/src/physics/collision.rs
badd +49 src/main.rs
badd +11 src/physics/mod.rs
badd +42 ~/Dock/repos/rust/bevy/jump-game/src/physics/motion.rs
badd +1 ~/Dock/repos/rust/bevy/jump-game/src/physics/gravity.rs
badd +18 ~/Dock/repos/rust/bevy/jump-game/src/demo/player.rs
badd +3 oil:///home/nathaniel/Dock/repos/rust/bevy/jump-game/src/
badd +22 ~/Dock/repos/rust/bevy/jump-game/src/demo/animation.rs
badd +49 ~/Dock/repos/rust/bevy/jump-game/src/demo/movement.rs
badd +5 ~/Dock/repos/rust/bevy/jump-game/src/screens/gameplay.rs
badd +10 ~/Dock/repos/rust/bevy/jump-game/src/screens/mod.rs
badd +58 ~/Dock/repos/rust/bevy/jump-game/src/demo/level.rs
argglobal
%argdel
edit ~/Dock/repos/rust/bevy/jump-game/src/demo/level.rs
argglobal
balt ~/Dock/repos/rust/bevy/jump-game/src/physics/collision.rs
setlocal foldmethod=expr
setlocal foldexpr=nvim_treesitter#foldexpr()
setlocal foldmarker={{{,}}}
setlocal foldignore=#
setlocal foldlevel=5
setlocal foldminlines=1
setlocal foldnestmax=20
setlocal foldenable
let s:l = 67 - ((27 * winheight(0) + 14) / 28)
if s:l < 1 | let s:l = 1 | endif
keepjumps exe s:l
normal! zt
keepjumps 67
normal! 0
tabnext 1
if exists('s:wipebuf') && len(win_findbuf(s:wipebuf)) == 0 && getbufvar(s:wipebuf, '&buftype') isnot# 'terminal'
  silent exe 'bwipe ' . s:wipebuf
endif
unlet! s:wipebuf
set winheight=1 winwidth=20
let &shortmess = s:shortmess_save
let s:sx = expand("<sfile>:p:r")."x.vim"
if filereadable(s:sx)
  exe "source " . fnameescape(s:sx)
endif
let &g:so = s:so_save | let &g:siso = s:siso_save
doautoall SessionLoadPost
unlet SessionLoad
" vim: set ft=vim :
