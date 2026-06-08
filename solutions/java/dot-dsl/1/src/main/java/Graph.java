import java.util.*;

public class Graph {
    private final Map<String, String> attributes = new LinkedHashMap<>();
    private final Map<String, Node> nodes = new LinkedHashMap<>();
    private final List<Edge> edges = new ArrayList<>();
    public Graph() {
    }

    public Graph(Map<String, String> attributes) {
        if (attributes != null) {
            this.attributes.putAll(attributes);
        }
    }

    public Collection<Node> getNodes() {
        return nodes.values();
    }

    public Collection<Edge> getEdges() {
        return edges;
    }

    public Graph node(String name) {
        return node(name, Collections.emptyMap());
    }

    public Graph node(String name, Map<String, String> attributes) {
        if (!nodes.containsKey(name)) {
            nodes.put(name, new Node(name, attributes));
        } else {
            nodes.put(name, new Node(name, attributes));
        }
        return this;
    }

    public Graph edge(String start, String end) {
        return edge(start, end, Collections.emptyMap());
    }

    public Graph edge(String start, String end, Map<String, String> attributes) {
        edges.add(new Edge(start, end, attributes));
        return this;
    }

    public Map<String, String> getAttributes() {
        return attributes;
    }
}
