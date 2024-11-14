import { ComponentFixture, TestBed } from '@angular/core/testing';

import { VersionSelectComponent } from './version-select.component';

describe('VersionSelectComponent', () => {
  let component: VersionSelectComponent;
  let fixture: ComponentFixture<VersionSelectComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [VersionSelectComponent]
    })
    .compileComponents();

    fixture = TestBed.createComponent(VersionSelectComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
