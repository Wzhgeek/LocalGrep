import { describe, expect, it } from 'vitest';
import { SearchBar } from '../components/SearchBar';

describe('SearchBar component contract', () => {
  it('should be defined', () => {
    expect(SearchBar).toBeTypeOf('function');
  });
});
