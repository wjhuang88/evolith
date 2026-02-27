import { MainLayout } from '@/components/layout/MainLayout';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/Card';

export default function DashboardPage() {
  return (
    <MainLayout>
      <div className="space-y-6">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Dashboard</h1>
          <p className="text-muted-foreground">Welcome to Evolith - AI Agent Development Platform</p>
        </div>

        {/* Stats Grid */}
        <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
          <StatCard title="Total Tools" value="12" description="MCP tools available" />
          <StatCard title="Skills" value="8" description="Custom skills" />
          <StatCard title="Snippets" value="156" description="Code snippets" />
          <StatCard title="API Calls" value="1,234" description="This month" />
        </div>

        {/* Recent Activity */}
        <div className="grid gap-4 md:grid-cols-2">
          <Card>
            <CardHeader>
              <CardTitle>Recent Tools</CardTitle>
              <CardDescription>Recently added MCP tools</CardDescription>
            </CardHeader>
            <CardContent>
              <div className="space-y-4">
                <RecentItem name="filesystem_read" type="tool" />
                <RecentItem name="http_request" type="tool" />
                <RecentItem name="database_query" type="tool" />
              </div>
            </CardContent>
          </Card>

          <Card>
            <CardHeader>
              <CardTitle>Quick Actions</CardTitle>
              <CardDescription>Common tasks</CardDescription>
            </CardHeader>
            <CardContent>
              <div className="grid grid-cols-2 gap-4">
                <QuickActionButton label="Add Tool" href="/tools/new" />
                <QuickActionButton label="Create Skill" href="/skills/new" />
                <QuickActionButton label="Add Snippet" href="/snippets/new" />
                <QuickActionButton label="View Docs" href="/docs" />
              </div>
            </CardContent>
          </Card>
        </div>
      </div>
    </MainLayout>
  );
}

function StatCard({ title, value, description }: { title: string; value: string; description: string }) {
  return (
    <Card>
      <CardContent className="pt-6">
        <div className="flex flex-col">
          <span className="text-sm font-medium text-muted-foreground">{title}</span>
          <span className="text-3xl font-bold">{value}</span>
          <span className="text-xs text-muted-foreground">{description}</span>
        </div>
      </CardContent>
    </Card>
  );
}

function RecentItem({ name, type }: { name: string; type: string }) {
  return (
    <div className="flex items-center justify-between rounded-lg border p-3">
      <div className="flex items-center gap-3">
        <div className="flex h-8 w-8 items-center justify-center rounded bg-primary/10">
          <span className="text-xs font-medium text-primary">{type[0].toUpperCase()}</span>
        </div>
        <div>
          <p className="font-medium">{name}</p>
          <p className="text-xs text-muted-foreground">Added recently</p>
        </div>
      </div>
    </div>
  );
}

function QuickActionButton({ label, href }: { label: string; href: string }) {
  return (
    <a
      href={href}
      className="flex items-center justify-center rounded-lg border border-dashed p-4 text-sm font-medium transition-colors hover:border-primary hover:bg-primary/5"
    >
      {label}
    </a>
  );
}
