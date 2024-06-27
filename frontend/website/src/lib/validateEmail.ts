export function validateEmail(email: string): boolean {
	return /^[a-zA-Z\d_+&*-]+(?:\\.[a-zA-Z\d_+&*-]+)*@(?:[a-zA-Z\d-]+\.)+[a-zA-Z]{2,7}$/.test(email);
}
