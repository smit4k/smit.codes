const copiedLabel = 'Copied';
const defaultLabel = 'Copy';

function resetButton(button: HTMLButtonElement) {
	button.textContent = defaultLabel;
	button.removeAttribute('data-copied');
}

async function copyText(text: string) {
	if (navigator.clipboard?.writeText) {
		await navigator.clipboard.writeText(text);
		return;
	}

	const textarea = document.createElement('textarea');
	textarea.value = text;
	textarea.setAttribute('readonly', '');
	textarea.style.position = 'fixed';
	textarea.style.top = '-9999px';
	document.body.appendChild(textarea);
	textarea.select();
	document.execCommand('copy');
	textarea.remove();
}

export function installCodeBlockCopy(container: HTMLElement) {
	const timeouts = new WeakMap<HTMLButtonElement, number>();

	const onClick = async (event: MouseEvent) => {
		const target = event.target;
		if (!(target instanceof Element)) return;

		const button = target.closest<HTMLButtonElement>('.code-copy-button');
		if (!button || !container.contains(button)) return;

		const codeBlock = button.closest('.code-block');
		const code = codeBlock?.querySelector('pre code');
		const text = code?.textContent;
		if (!text) return;

		await copyText(text);

		const existingTimeout = timeouts.get(button);
		if (existingTimeout) {
			window.clearTimeout(existingTimeout);
		}

		button.textContent = copiedLabel;
		button.setAttribute('data-copied', 'true');
		timeouts.set(
			button,
			window.setTimeout(() => resetButton(button), 1600)
		);
	};

	container.addEventListener('click', onClick);

	return () => {
		container.removeEventListener('click', onClick);
	};
}
