import React from 'react';
import { render, screen } from '@testing-library/react';
import App from './App';

test('renders learn react link', () => {
  render(<App />);
  const buttonEl = screen.getByText(/click me/i);
  expect(buttonEl).toBeInTheDocument();
});
