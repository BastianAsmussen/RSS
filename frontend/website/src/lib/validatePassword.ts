export function validatePassword(password: string) {
	const length = password.length >= 8 && password.length <= 128;
	const upperLower = /[a-zA-Z]/.test(password);
	const lettersNumbers = /[a-zA-Z]\d/.test(password);
	const specialChar = /[!@#?\]]/.test(password) && !/[<>]/.test(password);

	return {
		length,
		upperLower,
		lettersNumbers,
		specialChar,
		all: length && upperLower && lettersNumbers && specialChar
	};
}
