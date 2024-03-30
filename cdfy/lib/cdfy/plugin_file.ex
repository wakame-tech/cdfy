defmodule Cdfy.PluginFile do
  alias ExAws.S3

  @bucket "plugins"

  def download(name) do
    S3.get_object(@bucket, name)
    |> ExAws.request!()
    |> then(& &1.body)
  end

  def upload(name, path) do
    S3.Upload.stream_file(path)
    |> S3.upload(@bucket, name)
    |> ExAws.request()
  end
end
