window.onload = () => main();

function main(): void {
    const table = document.getElementById("report") as HTMLTableElement;
    let data = new Map<string, number>();
    for (let row of table.rows) {
        data.set(row.cells[0].innerText, parseFloat(row.cells[1].innerText));
    }
    drawGraph(data);
}

function drawGraph(data: Map<string, number>): void {
    const svg = document.getElementById("graph") as (SVGElement & HTMLElement);
    drawLine(svg, { start: { x: 0, y: 0 }, end: { x: 0, y: 100 } });
    drawLine(svg, { start: { x: 0, y: 50 }, end: { x: 300, y: 50 } });

    const extremes: Extremes = findExtremes(Array.from(data.values()));
    const scale = 50 / Math.max(Math.abs(extremes.min), Math.abs(extremes.max));
    const barWidth = 15;
    let index = 0;
    for (let val of data.values()) {
        const barHeight = Math.abs(val) * scale;
        drawRectangle(svg,
            {
                origin: {
                    x: index * barWidth,
                    y: val > 0 ? 50 - barHeight : 50
                },
                width: barWidth,
                height: barHeight
            },
            val > 0 ? 'green' : 'red'
        );
        index++;
    }
}

interface Extremes {
    min: number;
    max: number;
}

function findExtremes(data: Array<number>): Extremes {
    let extremes: Extremes = { min: Number.MAX_VALUE, max: Number.MIN_VALUE };
    for (let value of data) {
        if (value <= extremes.min) {
            extremes.min = value;
        }
        if (value >= extremes.max) {
            extremes.max = value;
        }
    }
    return extremes;
}

type Point = { x: number, y: number };

interface Line {
    start: Point;
    end: Point;
}

function drawLine(svg: SVGElement, line: Line): void {
    const svgLine = document.createElementNS("http://www.w3.org/2000/svg", "line") as SVGLineElement;

    svgLine.setAttribute('x1', line.start.x.toString());
    svgLine.setAttribute('y1', line.start.y.toString());
    svgLine.setAttribute('x2', line.end.x.toString());
    svgLine.setAttribute('y2', line.end.y.toString());
    svgLine.setAttribute('stroke', 'black');
    svgLine.setAttribute('strokeWidth', '5px');

    svg.appendChild(svgLine);
}

interface Rectangle {
    origin: Point;
    width: number;
    height: number;
}

function drawRectangle(svg: SVGElement, rect: Rectangle, color: string): void {
    const svgRect = document.createElementNS("http://www.w3.org/2000/svg", "rect") as SVGRectElement;

    svgRect.setAttribute('x', rect.origin.x.toString());
    svgRect.setAttribute('y', rect.origin.y.toString());
    svgRect.setAttribute('width', rect.width.toString());
    svgRect.setAttribute('height', rect.height.toString());
    svgRect.setAttribute('fill', color);

    svg.appendChild(svgRect);
}
