window.onload = () => main();

function main(): void {
    const table = document.getElementById("report") as HTMLTableElement;
    let data: { [tag: string]: number } = {};
    for (let row of table.rows) {
        data[row.cells[0].innerText] = parseFloat(row.cells[1].innerText);
    }
    drawGraph(data);
}

function drawGraph(data: { [tag: string]: number }): void {
    const svg = document.getElementById("graph") as (SVGElement & HTMLElement);
    drawLine(svg, { start: { x: 0, y: 0 }, end: { x: 0, y: 100 } });
    drawLine(svg, { start: { x: 0, y: 50 }, end: { x: 300, y: 50 } });
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
