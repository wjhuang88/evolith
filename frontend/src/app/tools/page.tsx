import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui';
import { Button } from '@/components/ui';

export default function ToolsPage() {
  return (
    <div className="container mx-auto py-8">
      <div className="flex items-center justify-between mb-8">
        <div>
          <h1 className="text-3xl font-bold">MCP Tools</h1>
          <p className="text-muted-foreground mt-1">
            标准化的工具封装，通过MCP协议提供服务
          </p>
        </div>
        <Button>Create Tool</Button>
      </div>

      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
        {/* Placeholder cards */}
        {[1, 2, 3].map((i) => (
          <Card key={i}>
            <CardHeader>
              <CardTitle>Tool {i}</CardTitle>
              <CardDescription>
                A sample tool description that explains what this tool does.
              </CardDescription>
            </CardHeader>
            <CardContent>
              <div className="flex items-center justify-between text-sm text-muted-foreground">
                <span>Public</span>
                <span>v1.0.0</span>
              </div>
            </CardContent>
          </Card>
        ))}
      </div>

      <div className="mt-8 text-center text-muted-foreground">
        <p>No tools available yet. Create your first tool to get started.</p>
      </div>
    </div>
  );
}
