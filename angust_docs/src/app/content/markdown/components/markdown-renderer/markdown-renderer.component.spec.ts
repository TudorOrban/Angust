import { ComponentFixture, TestBed } from '@angular/core/testing';

import { MarkdownRendererComponent } from './markdown-renderer.component';

describe('MarkdownRendererComponent', () => {
  let component: MarkdownRendererComponent;
  let fixture: ComponentFixture<MarkdownRendererComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [MarkdownRendererComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(MarkdownRendererComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
