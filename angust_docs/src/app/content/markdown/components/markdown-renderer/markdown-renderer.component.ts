import { Component, Input } from '@angular/core';

@Component({
  selector: 'app-markdown-renderer',
  standalone: true,
  imports: [],
  templateUrl: './markdown-renderer.component.html',
  styleUrl: './markdown-renderer.component.css'
})
export class MarkdownRendererComponent {
    @Input() fileContent?: string;
}
