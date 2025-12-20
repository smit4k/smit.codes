export function formatDate(dateStr: string): string | null {
    const months: string[] = [
        "January", "February", "March", "April", "May", "June",
        "July", "August", "September", "October", "November", "December"
    ];

    const [year, month, day] = dateStr.split('-');
    const monthIndex = parseInt(month) - 1;

    return `${months[monthIndex]} ${parseInt(day)}, ${year}`;
}
