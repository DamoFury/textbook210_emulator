cargo fmt --all
trunk build --release

git stage *
git commit -m "fmt"
git push

cp -r dist/* ../210_web_fella/
cd ../210_web_fella/

git stage *
git commit -m "update website"
git push
