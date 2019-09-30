# Alias Extractor

simply reads an alias from stdin and spits out the alias

Example:

```
    echo "kaf='kubectl apply -f' | alias_extractor
    > kaf
```

    alias_extractor

    Usage:
        <stdin> | alias_extractor

        where <stdin> is in the form of

        <string>=<command>

        just like one would use alias <string>=<command>

        Example:

        alias kaf='kubectl apply -f'
        alias | head -1 | alias_extractor
        > kaf
        
