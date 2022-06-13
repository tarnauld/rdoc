pub fn template_css<'style>() -> &'style str {
    ":root {
        --primary: #4C5454;
        --primary-dark: #c48b9f;
        --primary-light: #ffeeff;
        --secondary: #ffd54f;
        --secondary-dark: #c8a415;
        --secondary-light: #ffff81;
        --fill-dark: #E1E2E1;
        --fill-light: #F5F5F6;
        --shadow: #c6c6c6;
    }
    
    html,
    body {
        width: 100%;
        height: 100%;
        overflow: auto;
        margin: 0;
        background-color: var(--fill-dark);
        font-family: Arial, Helvetica, sans-serif;
    }
    
    header {
        background-color: var(--primary-dark);
        height: 60px;
        display: flex;
        justify-content: center;
        align-items: center;
    }

    header > a {
        text-decoration: none;
        font-size: 2em;
        color: var(--primary);
    }
    
    .search {
        background-color: var(--primary-light);
        height: 50px;
    }
    
    .items-container {
        height: fit-content;
        padding: 1.5em;
        display: grid;
        grid-gap: 15px;
    }
    
    .item {
        background-color: var(--fill-light);
        border-radius: 5px;
        box-shadow: 2px 5px 10px var(--shadow);
        display: flex;
        justify-content: space-between;
    }

    .item > .commit {
        padding: 20px;
    }

    .item > .commit > div {
        margin-top: 6px;
    }
    
    .item .title {
        font-size: 1.5em;
        font-weight: 600;
        color: #444;
    }
    
    .item .metadatas {
        color: var(--primary);
        font-size: 0.5em;
    }
    
    .item .authors {
        display: flex;
        flex-direction: row;
    }
    
    .author {
        background-color: var(--secondary);
        width: fit-content;
        padding: 5px;
        border-radius: 10px;
        margin-right: 6px;
    }

    .tag {
        background-color: var(--secondary);
        width: fit-content;
        padding: 5px;
        border-radius: 10px;
        margin-right: 6px;
    }
    
    .item .content {
        color: var(--primary);
        font-size: 0.8em;
    }

    .go {
        background-color: var(--secondary-dark);
        width: 200px;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    .go > a {
        text-decoration: none;
        font-size: 2em;
        color: var(--fill-light);
    }
    
    .footer {
        content: '';
        height: 50px;
        background-color: var(--primary-dark);
    }

    .commit-container {
        height: 100%;
        margin: 20px;
        background-color: var(--fill-light);
        border-radius: 3px;
        box-shadow: 2px 5px 10px #c6c6c6;
        padding: 20px;
    }

    .commit-id {
        display: flex;
        align-items: center;
    }

    .commit-id > h1 {
        overflow-wrap: anywhere;
    }

    .commit-container > .authors, .tags {
        display: flex;
        align-items: center;
        font-size: 0.6em;
        margin-top: 6px;
    }

    .commit-container > .authors > span, .tags > span {
        margin-right: 6px;
    }
    "
}
