import { Injectable } from '@angular/core';
import { marked } from 'marked';
import hljs from 'highlight.js';

@Injectable({
    providedIn: 'root',
})
export class MarkdownRendererService {
    constructor() {
        const renderer = new marked.Renderer();
        renderer.code = (data) => {
            const language = hljs.getLanguage(data.lang ?? "plaintext") ? data.lang : 'plaintext';
            return `<pre><code class="hljs ${language}">${
                hljs.highlight(data.text ?? "", { language: language ?? "" }).value
            }</code></pre>`;
        };

        marked.setOptions({
            renderer: renderer,
        });
    }

    renderMarkdown(markdown: string): string | Promise<string> {
        return marked(markdown);
    }
}