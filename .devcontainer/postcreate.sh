sudo ln -snf /usr/share/zoneinfo/Europe/Amsterdam /etc/localtime && echo 'Europe/Amsterdam' | sudo tee /etc/timezone

git config --global user.name 'Bas Netten'
git config --global user.email 'bas.netten.da@outlook.com'
git config --global credential.useHttpPath true

git config --global alias.ci "commit"
git config --global alias.co "checkout"
git config --global alias.br "branch"
git config --global alias.cian "commit --amend --no-edit"
git config --global alias.st "status"
git config --global alias.loga "log --oneline --decorate --graph --all"
git config --global alias.capf '!git add . && git commit --amend --no-edit && git push --force-with-lease'

curl -o ~/.git-completion.bash https://raw.githubusercontent.com/git/git/master/contrib/completion/git-completion.bash
completion_snippet='
# Git tab completion
if [ -f ~/.git-completion.bash ]; then
    . ~/.git-completion.bash
fi
'

# Check if the snippet already exists in .bashrc
if ! grep -q "git-completion.bash" ~/.bashrc; then
    # Append the git-completion snippet to .bashrc
    echo "$completion_snippet" >> ~/.bashrc
    echo "Git completion added to .bashrc"
else
    echo "Git completion is already configured in .bashrc"
fi

source ~/.bashrc
