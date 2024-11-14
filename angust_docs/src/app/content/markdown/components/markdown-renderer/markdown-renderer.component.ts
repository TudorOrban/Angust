import { Component, Input, OnChanges, SimpleChanges } from '@angular/core';
import { MarkdownRendererService } from '../../services/markdown-renderer.service';

@Component({
    selector: 'app-markdown-renderer',
    standalone: true,
    imports: [],
    templateUrl: './markdown-renderer.component.html',
    styleUrl: './markdown-renderer.component.css',
})
export class MarkdownRendererComponent implements OnChanges {
    @Input() fileContent?: string;
    renderedContent?: string;

    constructor(
        private readonly markdownRendererService: MarkdownRendererService
    ) {}

    ngOnChanges(changes: SimpleChanges): void {
        if (!changes['fileContent'] || !this.fileContent) {
            return;
        }

        const result = this.markdownRendererService.renderMarkdown(
            this.fileContent
        );
        if (!(result instanceof Promise)) {
            console.log("Result: ", result)
            this.renderedContent = result;
            return;
        }

        result
            .then((renderedContent) => {
                this.renderedContent = renderedContent;
            })
            .catch((error) => {
                console.error('Error rendering markdown:', error);
                this.renderedContent = 'Error: Unable to render markdown.';
            });
    }
}
