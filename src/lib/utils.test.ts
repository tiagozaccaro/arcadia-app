import { describe, expect, it } from 'vitest';
import { cn } from './utils';

describe('cn', () => {
  it('merges class names correctly', () => {
    expect(cn('class1', 'class2')).toBe('class1 class2');
  });

  it('handles conditional classes', () => {
    const shouldAddClass2 = true;
    const shouldAddClass3 = false;
    expect(
      cn(
        'class1',
        shouldAddClass2 ? 'class2' : '',
        shouldAddClass3 ? 'class3' : ''
      )
    ).toBe('class1 class2');
  });

  it('merges Tailwind classes', () => {
    expect(cn('px-2 py-1', 'px-4')).toBe('py-1 px-4');
  });
});
