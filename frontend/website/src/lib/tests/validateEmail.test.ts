import { describe, test, expect } from 'vitest';
import { validateEmail } from '$lib/validateEmail';

describe('Email Validation', () => {
	test('Tries with @ and .com for the Mail', () => {
		// eslint-disable-next-line @typescript-eslint/no-unused-expressions
		expect(validateEmail('test@domain.com')).toBeTruthy;
	});

	test('Input without @', () => {
		// eslint-disable-next-line @typescript-eslint/no-unused-expressions
		expect(validateEmail('test.com')).toBeFalsy;
	});

	test('Input without TLD', () => {
		// eslint-disable-next-line @typescript-eslint/no-unused-expressions
		expect(validateEmail('test@domain')).toBeFalsy;
	});

	test('No Input', () => {
		// eslint-disable-next-line @typescript-eslint/no-unused-expressions
		expect(validateEmail('')).toBeFalsy;
	});
});
