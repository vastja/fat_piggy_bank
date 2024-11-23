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
    const svg = document.getElementById("graph") as (SVGElement & SVGFitToViewBox & HTMLElement);
    const graphInfo: GraphInfo = drawCoordinateSystemForData(svg, Array.from(data.values()));

    const space = 1;
    const barWidth = (graphInfo.range.space - (space * data.size)) / data.size;

    let index = 0;
    for (let val of data.values()) {
        const barHeight = Math.abs(val) * graphInfo.valueScale;
        drawRectangle(svg,
            {
                origin: {
                    x: space + index * (barWidth + space),
                    y: val > 0 ? graphInfo.baseline - barHeight : graphInfo.baseline
                },
                width: barWidth,
                height: barHeight
            },
            val > 0 ? 'green' : 'red'
        );
        index++;
    }
}

interface GraphInfo {
    range: { value: number; space: number };
    baseline: number;
    valueScale: number;
}

function drawCoordinateSystemForData(svg: SVGElement & SVGFitToViewBox, data: Array<number>): GraphInfo {
    const baseline = svg.viewBox.baseVal.height * 0.5;

    drawLine(svg, { start: { x: 0, y: 0 }, end: { x: 0, y: svg.viewBox.baseVal.height } });
    drawLine(svg, { start: { x: 0, y: baseline }, end: { x: svg.viewBox.baseVal.width, y: baseline } });

    const extremes: Extremes = findExtremes(data);
    const valueRange = Math.max(Math.abs(extremes.min), Math.abs(extremes.max));
    const valueScale = baseline / valueRange;

    drawText(svg, valueRange.toString(), { x: 0, y: 0 });

    return {
        range: { value: valueRange, space: svg.viewBox.baseVal.width }, baseline, valueScale
    };
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
    svgLine.setAttribute('stroke-width', '1px');

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

function drawText(svg: SVGElement, text: string, origin: Point): void {
    const svgText = document.createElementNS("http://www.w3.org/2000/svg", "text") as SVGTextElement;

    svgText.setAttribute('x', origin.x.toString());
    svgText.setAttribute('y', origin.y.toString());
    svgText.innerHTML = text;
    svgText.setAttribute('alignment-baseline', 'hanging');
    svgText.classList.add('small');

    svg.appendChild(svgText);
}
