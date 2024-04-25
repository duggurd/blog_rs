gh_md_endpoint="https://api.github.com/markdown"
blog_path="./templates/articles"

articles=$(ls "$blog_path")

for article in templates/articles/*.md; do 

    content=$(cat "$article")
    name=$(echo $article | cut -d. -f1)
    

    json=$(jq --arg c "$content" -n '{text: $c}')

    curl -L \
        -X POST \
        -H "Accept: application/vnd.github+json" \
        -H "Authorization: Bearer $github_pat" \
        -H "X-GitHub-Api-Version: 2022-11-28" \
        https://api.github.com/markdown \
        -d "$json" \
        -o "$name".html
done