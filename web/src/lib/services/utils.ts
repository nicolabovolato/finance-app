export const truncate = (str: string, size: number) =>
	str.length > size ? str.substring(0, size) + '...' : str;

export const getDateFormatter = () =>
	new Intl.DateTimeFormat('en-US', {
		month: 'short',
		day: 'numeric',
	});

export const getCurrencyFormatter = (
	currency: Intl.NumberFormatOptions['currency'],
	signDisplay?: Intl.NumberFormatOptions['signDisplay'],
) =>
	new Intl.NumberFormat('en-US', {
		style: 'currency',
		currency,
		signDisplay,
	});

export const transformCustomEvent = (event: CustomEvent, name: string) =>
	new CustomEvent('change', {
		detail: {
			value: event.detail,
			name,
		},
	});
