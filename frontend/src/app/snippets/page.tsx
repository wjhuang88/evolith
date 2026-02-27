import { Card, CardHeader, CardTitle, CardDescription, CardContent } from '@/components/ui';
import { Button, Input } from '@/components/ui';

export default function SnippetsPage() {
  return (
    <div className="container mx-auto py-8">
      <div className="flex items-center justify-between mb-8">
        <div>
          <h1 className="text-3xl font-bold">Code Snippets</h1>
          <p className="text-muted-foreground mt-1">
            面向大模型的代码片段仓库，降低token消耗
          </p>
        </div>
        <Button>Create Snippet</Button>
      </div>

      <div className="mb-6">
        <Input type="search" placeholder="Search snippets..." className="max-w-md" />
      </div>

      <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
        {/* Placeholder cards */}
        {[1, 2, 3, 4, 5, 6].map((i) => (
          <Card key={i}>
            <CardHeader>
              <CardTitle>Snippet {i}</CardTitle>
              <CardDescription>
                A sample snippet description with usage examples.
              </CardDescription>
            </CardHeader>
            <CardContent>
              <div className="flex items-center justify-between text-sm text-muted-foreground">
                <span>TypeScript</span>
                <span>~150 tokens</span>
              </div>
            </CardContent>
          </Card>
        ))}
      </div>
    </div>
  );
}
